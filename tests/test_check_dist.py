"""Task T-13h (c), ADR-57 item 3 and ADR-59 item 5 (c): ``tests/check_dist.py`` rejects what ADR-57 lists.

Small synthetic archives are built in ``tmp_path`` (names and versions from the root ``Cargo.toml`` through
``check_dist.pep440``; the real archives are not needed). One FAIL per rule, and a clean pair that passes, so that
each rule is a mutation a test catches."""

from __future__ import annotations

import io
import tarfile
import zipfile
from collections.abc import Callable
from pathlib import Path

import pytest
from check_dist import cargo_version, check_sdist, check_wheel, declared_project_urls, main, pep440

LICENSES = ("LICENSE-MIT", "LICENSE-APACHE")
SDIST_NEEDED = (
    "Cargo.toml",
    "Cargo.lock",
    "pyproject.toml",
    "README.md",
    "LICENSE-MIT",
    "LICENSE-APACHE",
    "crates/oradmp-core/src/lib.rs",
    "crates/oradmp-core/data/exp_intro_lines.txt",
    "crates/oradmp-py/src/lib.rs",
    "python/oradmp/__init__.py",
    "python/oradmp/_cli.py",
)
# T-20 (b): the literal addresses (not read from pyproject.toml, so that a test can tell the file from the rule)
SOURCE_URL = "https://github.com/granddad-wolf/oradmp"
ISSUES_URL = "https://github.com/granddad-wolf/oradmp/issues"
ENTRY_POINTS = "[console_scripts]\noradmp = oradmp._cli:main\n"


def version() -> str:
    v = pep440(cargo_version())
    assert v is not None
    return v


def metadata(v: str, description: str = "", edit: Callable[[str], str] | None = None) -> bytes:
    """``edit``: changes the text (one property broken) for the T-13j cases m12 to m21."""
    text = (
        "Metadata-Version: 2.4\n"
        "Name: oradmp\n"
        f"Version: {v}\n"
        "Requires-Python: >=3.11\n"
        "Requires-Dist: pyarrow>=15\n"
        "License-Expression: MIT OR Apache-2.0\n"
        "License-File: LICENSE-MIT\n"
        "License-File: LICENSE-APACHE\n"
        f"Project-URL: Source, {SOURCE_URL}\n"
        f"Project-URL: Issues, {ISSUES_URL}\n"
        "\n"
        "# oradmp\n"
        f"{description}"
    )
    return (edit(text) if edit else text).encode()


def make_wheel(
    tmp_path: Path,
    extra: dict[str, bytes] | None = None,
    *,
    name_version: str | None = None,
    meta_version: str | None = None,
    description: str = "",
    meta_edit: Callable[[str], str] | None = None,
    omit: tuple[str, ...] = (),
) -> Path:
    """``name_version``: the version in the file name and the .dist-info folder; ``meta_version``: the METADATA's;
    ``meta_edit``: a change of the METADATA text; ``omit``: member names (below the .dist-info folder for the
    ``.dist-info/...`` ones, written ``DI/...``) that are left out."""
    v = name_version or version()
    path = tmp_path / f"oradmp-{v}-cp311-abi3-manylinux_2_28_x86_64.whl"
    di = f"oradmp-{v}.dist-info"
    members: dict[str, bytes] = {
        "oradmp/__init__.py": b"x = 1\n",
        "oradmp/_core.abi3.so": b"\x7fELF binary without paths\n",
        f"{di}/METADATA": metadata(meta_version or v, description, meta_edit),
        f"{di}/entry_points.txt": ENTRY_POINTS.encode(),
        f"{di}/licenses/LICENSE-MIT": b"MIT\n",
        f"{di}/licenses/LICENSE-APACHE": b"Apache\n",
        f"{di}/sboms/oradmp-py.cyclonedx.json": b'{"x": "path+file:///var/tmp/oradmp-release/src/crates/oradmp-py"}\n',
    }
    members.update(extra or {})
    for gone in omit:
        del members[gone.replace("DI", di, 1)]
    with zipfile.ZipFile(path, "w") as z:
        for name, data in members.items():
            z.writestr(name, data)
    return path


def make_sdist(
    tmp_path: Path,
    extra: dict[str, bytes] | None = None,
    *,
    root_name: str | None = None,
    file_name: str | None = None,
    omit: tuple[str, ...] = (),
) -> Path:
    """``root_name``: the root folder inside the archive (default ``oradmp-<version>``); ``file_name``: the archive's
    file name (default ``oradmp-<version>.tar.gz``); ``omit``: members (below the root folder) left out."""
    v = version()
    root = root_name or f"oradmp-{v}"
    path = tmp_path / (file_name or f"oradmp-{v}.tar.gz")
    members: dict[str, bytes] = {f"{root}/{n}": b"x\n" for n in SDIST_NEEDED}
    members[f"{root}/PKG-INFO"] = metadata(v)
    members.update({f"{root}/{n}": d for n, d in (extra or {}).items()})
    for gone in omit:
        del members[f"{root}/{gone}"]
    with tarfile.open(path, "w:gz") as t:
        for name, data in members.items():
            info = tarfile.TarInfo(name)
            info.size = len(data)
            t.addfile(info, io.BytesIO(data))
    return path


def wheel_errors(wheel: Path) -> list[str]:
    errors: list[str] = []
    check_wheel(str(wheel), errors)
    return errors


def sdist_errors(sdist: Path) -> list[str]:
    errors: list[str] = []
    check_sdist(str(sdist), errors, version())
    return errors


def test_a_clean_synthetic_pair_passes(tmp_path: Path, capsys: pytest.CaptureFixture[str]) -> None:
    wheel, sdist = make_wheel(tmp_path), make_sdist(tmp_path)
    assert wheel_errors(wheel) == []
    assert sdist_errors(sdist) == []
    assert main([str(wheel), str(sdist)]) == 0
    assert "check_dist: ok" in capsys.readouterr().out


def test_a_pyc_in_the_wheel_fails(tmp_path: Path) -> None:
    w = make_wheel(tmp_path, {"oradmp/__pycache__/_api.cpython-313.pyc": b"\x00pyc"})
    errors = wheel_errors(w)
    assert any("__pycache__" in e and "_api.cpython-313.pyc" in e for e in errors), errors


def test_a_pyo_in_the_wheel_fails(tmp_path: Path) -> None:
    errors = wheel_errors(make_wheel(tmp_path, {"oradmp/x.pyo": b"\x00"}))
    assert any("oradmp/x.pyo" in e and "compiled Python file" in e for e in errors), errors


def test_a_corpus_folder_nested_in_the_sdist_fails(tmp_path: Path) -> None:
    errors = sdist_errors(make_sdist(tmp_path, {"python/oradmp/corpus/x.txt": b"x\n"}))
    assert any("python/oradmp/corpus/x.txt" in e and "corpus/" in e for e in errors), errors


def test_a_tools_folder_in_the_sdist_fails(tmp_path: Path) -> None:
    errors = sdist_errors(make_sdist(tmp_path, {"tools/release_build.sh": b"#!/bin/sh\n"}))
    assert any("tools/release_build.sh" in e and "tools/" in e for e in errors), errors


@pytest.mark.parametrize("name", ["tests", "docs", "target", ".venv", ".git"])
def test_every_forbidden_folder_fails_at_any_depth(tmp_path: Path, name: str) -> None:
    errors = sdist_errors(make_sdist(tmp_path, {f"crates/oradmp-py/{name}/x.txt": b"x\n"}))
    assert any(f"{name}/" in e for e in errors), errors
    errors = wheel_errors(make_wheel(tmp_path, {f"oradmp/sub/{name}/x.txt": b"x\n"}))
    assert any(f"{name}/" in e for e in errors), errors


def test_a_home_path_in_the_sbom_fails_with_member_offset_and_line(tmp_path: Path) -> None:
    sbom = b'{\n "ref": "path+file:///home/u/p/oradmp/crates/oradmp-py"\n}\n'
    errors = wheel_errors(make_wheel(tmp_path, {f"oradmp-{version()}.dist-info/sboms/oradmp-py.cyclonedx.json": sbom}))
    hits = [e for e in errors if "sboms/oradmp-py.cyclonedx.json" in e and "/home/" in e]
    assert len(hits) == 1, errors
    assert f"byte offset {sbom.find(b'/home/')} (line 2)" in hits[0]


def test_a_users_path_inside_a_binary_member_fails(tmp_path: Path) -> None:
    errors = wheel_errors(make_wheel(tmp_path, {"oradmp/_core.abi3.so": b"\x00\x01panic at /Users/u/.cargo/x.rs\x00"}))
    assert any("_core.abi3.so" in e and "/Users/" in e for e in errors), errors


def test_a_root_home_in_the_sdist_fails(tmp_path: Path) -> None:
    errors = sdist_errors(make_sdist(tmp_path, {"README.md": b"see /root/work\n"}))
    assert any("README.md" in e and "/root/" in e for e in errors), errors


@pytest.mark.parametrize("text", [b'"C:\\\\Users\\\\u"', b"C:\\Users\\u", b"c:\\USERS\\u"])
def test_a_windows_home_fails_plain_escaped_and_in_any_case(tmp_path: Path, text: bytes) -> None:
    errors = wheel_errors(make_wheel(tmp_path, {"oradmp/info.json": b"{" + text + b"}"}))
    assert any("info.json" in e and "home path" in e for e in errors), errors


def test_the_test_probe_in_the_wheels_extension_module_fails(tmp_path: Path) -> None:
    # ADR-59 item 5 (c): a release built with --features test-probes
    errors = wheel_errors(make_wheel(tmp_path, {"oradmp/_core.abi3.so": b"\x7fELF...\x00_probe_panic\x00..."}))
    assert any("_core.abi3.so" in e and "_probe_panic" in e for e in errors), errors
    # the stub file names it too and is fine: the rule looks at the binary only
    assert wheel_errors(make_wheel(tmp_path, {"oradmp/_core.pyi": b"def _probe_panic(): ...\n"})) == []


# --- T-13i (d), S-4, ADR-64 item 4: the older rules, one synthetic case each (m1 to m9). Each case asserts exactly
# one FAIL line that names its rule, so a rule removed alone fails exactly its own test.


def only(errors: list[str], rule: str) -> None:
    assert len(errors) == 1, errors
    assert rule in errors[0], errors


def test_m1_a_wheel_named_with_another_version_fails(tmp_path: Path) -> None:
    only(wheel_errors(make_wheel(tmp_path, name_version="0.0.0", meta_version="0.0.0")), "from Cargo.toml")


def test_m2_a_metadata_version_other_than_the_wheel_name_fails(tmp_path: Path) -> None:
    only(wheel_errors(make_wheel(tmp_path, meta_version="0.0.0")), "Version is")


def test_m3_the_old_project_name_fails_unless_it_is_the_allowed_line(tmp_path: Path) -> None:
    old = "ora" + "dump"  # built from parts, as check_dist.py does (ADR-54)
    only(wheel_errors(make_wheel(tmp_path, {"oradmp/_api.py": f"# was {old}\n".encode()})), "the old project name")
    # the same pair with only the allowed line in METADATA passes
    allowed = (
        "oradmp (not " + old + ", which is an unrelated project on PyPI): "
        '"dmp" as in Oracle dump files, legacy exp now and Data Pump later.\n'
    )
    assert wheel_errors(make_wheel(tmp_path, description=allowed)) == []
    # and the same line anywhere else is not allowed
    only(wheel_errors(make_wheel(tmp_path, {"oradmp/_api.py": allowed.encode()})), "the old project name")


def test_m4_a_dump_file_in_the_sdist_fails(tmp_path: Path) -> None:
    only(sdist_errors(make_sdist(tmp_path, {"python/oradmp/x.dmp": b"x\n"})), "a dump file")


def test_m5_a_truth_file_in_the_sdist_fails(tmp_path: Path) -> None:
    only(sdist_errors(make_sdist(tmp_path, {"python/oradmp/truth_x.txt": b"x\n"})), "a truth file")


def test_m6_the_survey_tool_in_the_sdist_fails(tmp_path: Path) -> None:
    only(sdist_errors(make_sdist(tmp_path, {"python/exp_survey.py": b"x = 1\n"})), "the survey tool")


def test_m7_a_wheel_member_outside_oradmp_fails(tmp_path: Path) -> None:
    only(wheel_errors(make_wheel(tmp_path, {"other/x.py": b"x = 1\n"})), "outside oradmp/")


def test_m8_an_sdist_with_the_wrong_root_folder_fails(tmp_path: Path) -> None:
    only(sdist_errors(make_sdist(tmp_path, root_name="oradmp-0.0.0")), "root folder")


def test_m9_a_pycache_folder_fails_by_its_component_without_the_suffix_rule(tmp_path: Path) -> None:
    only(wheel_errors(make_wheel(tmp_path, {"oradmp/__pycache__/x.txt": b"x\n"})), "under a __pycache__/ folder")


# --- T-13j (e), review 05 SF-3, ADR-71 item 1: the other rules of check_dist.py, one synthetic case each (m10 to m21).
# Each case asserts the exact FAIL text and nothing else, so a rule removed alone fails exactly its own case.


def test_m10_an_sdist_with_another_file_name_fails(tmp_path: Path) -> None:
    v = version()
    errors = sdist_errors(make_sdist(tmp_path, file_name="oradmp-0.0.0.tar.gz"))
    assert errors == [f"sdist: file name 'oradmp-0.0.0.tar.gz' is not oradmp-{v}.tar.gz"]


def test_m11_the_old_name_in_an_sdist_member_fails_and_its_allowed_line_passes_once(tmp_path: Path) -> None:
    old = "ora" + "dump"
    root = f"oradmp-{version()}"
    errors = sdist_errors(make_sdist(tmp_path, {"python/oradmp/x.py": f"# {old}\n".encode()}))
    assert errors == [f"sdist member {root}/python/oradmp/x.py: the old project name at byte offset 2 (line 1)"]
    allowed = (
        "oradmp (not " + old + ", which is an unrelated project on PyPI): "
        '"dmp" as in Oracle dump files, legacy exp now and Data Pump later.\n'
    ).encode()
    assert sdist_errors(make_sdist(tmp_path, {"README.md": allowed})) == []
    # the allowed line is allowed in README.md and PKG-INFO only
    errors = sdist_errors(make_sdist(tmp_path, {"python/oradmp/x.py": allowed}))
    assert len(errors) == 1 and errors[0].startswith(f"sdist member {root}/python/oradmp/x.py: the old"), errors
    # the allowed line is exact: another line with the name in the same README fails
    errors = sdist_errors(make_sdist(tmp_path, {"README.md": allowed + f"also {old}\n".encode()}))
    assert len(errors) == 1 and errors[0].startswith(f"sdist member {root}/README.md: the old project name"), errors


def test_m12_requires_dist_without_pyarrow_15_fails(tmp_path: Path) -> None:
    w = make_wheel(tmp_path, meta_edit=lambda t: t.replace("pyarrow>=15", "pyarrow>=14"))
    assert wheel_errors(w) == ["wheel METADATA: Requires-Dist ['pyarrow>=14'] lacks pyarrow>=15"]


def test_m13_another_license_expression_fails(tmp_path: Path) -> None:
    w = make_wheel(tmp_path, meta_edit=lambda t: t.replace("MIT OR Apache-2.0", "MIT"))
    assert wheel_errors(w) == ["wheel METADATA: License-Expression is 'MIT'"]


def test_m14_another_requires_python_fails(tmp_path: Path) -> None:
    w = make_wheel(tmp_path, meta_edit=lambda t: t.replace(">=3.11", ">=3.9"))
    assert wheel_errors(w) == ["wheel METADATA: Requires-Python is '>=3.9'"]


@pytest.mark.parametrize("need", SDIST_NEEDED)
def test_m15_each_required_sdist_file_missing_fails(tmp_path: Path, need: str) -> None:
    assert sdist_errors(make_sdist(tmp_path, omit=(need,))) == [f"sdist: {need} is missing"]


@pytest.mark.parametrize("lic", LICENSES)
def test_m16_each_licence_file_missing_from_the_wheel_fails(tmp_path: Path, lic: str) -> None:
    errors = wheel_errors(make_wheel(tmp_path, omit=(f"DI/licenses/{lic}",)))
    assert errors == [f"wheel: {lic} is not under .dist-info/licenses/"]


def test_m17_metadata_version_2_3_fails(tmp_path: Path) -> None:
    w = make_wheel(tmp_path, meta_edit=lambda t: t.replace("Metadata-Version: 2.4", "Metadata-Version: 2.3"))
    assert wheel_errors(w) == ["wheel METADATA: Metadata-Version 2.3 is below 2.4"]


def test_m18_wrong_license_file_lines_fail(tmp_path: Path) -> None:
    w = make_wheel(tmp_path, meta_edit=lambda t: t.replace("License-File: LICENSE-APACHE\n", ""))
    assert wheel_errors(w) == ["wheel METADATA: License-File lines are ['LICENSE-MIT']"]


def test_m19_a_description_that_is_not_the_readme_fails(tmp_path: Path) -> None:
    w = make_wheel(tmp_path, meta_edit=lambda t: t.replace("# oradmp", "# something else"))
    assert wheel_errors(w) == ["wheel METADATA: the description is not the README"]


def test_m20_a_license_classifier_fails(tmp_path: Path) -> None:
    w = make_wheel(
        tmp_path, meta_edit=lambda t: t.replace("\n\n", "\nClassifier: License :: OSI Approved :: MIT License\n\n", 1)
    )
    assert wheel_errors(w) == ["wheel METADATA: a License :: classifier beside License-Expression"]


def test_m21_metadata_missing_from_the_wheel_and_pkg_info_from_the_sdist_fail(tmp_path: Path) -> None:
    v = version()
    assert wheel_errors(make_wheel(tmp_path, omit=("DI/METADATA",))) == [
        f"wheel: oradmp-{v}.dist-info/METADATA is missing"
    ]
    assert sdist_errors(make_sdist(tmp_path, omit=("PKG-INFO",))) == ["sdist: PKG-INFO is missing"]


# --- T-18 (j), ADR-72 item 11: the console script and the new required sdist file (cases m22 to m26).


def di() -> str:
    return f"oradmp-{version()}.dist-info"


def test_m22_a_wheel_without_entry_points_fails(tmp_path: Path) -> None:
    assert wheel_errors(make_wheel(tmp_path, omit=("DI/entry_points.txt",))) == [
        f"wheel: {di()}/entry_points.txt is missing"
    ]


@pytest.mark.parametrize(
    ("text", "message"),
    [
        (
            "[console_scripts]\noradmp = oradmp._cli:other\n",
            "[console_scripts] is [('oradmp', 'oradmp._cli:other')], not [('oradmp', 'oradmp._cli:main')]",
        ),
        (
            "[console_scripts]\noradmp = oradmp._cli:main\nextra = oradmp._cli:main\n",
            "[console_scripts] is [('oradmp', 'oradmp._cli:main'), ('extra', 'oradmp._cli:main')], "
            "not [('oradmp', 'oradmp._cli:main')]",
        ),
        (
            "[console_scripts]\noradmp = oradmp._cli:main\n[gui_scripts]\nx = y:z\n",
            "entry_points.txt sections are ['console_scripts', 'gui_scripts'], not ['console_scripts']",
        ),
        ("[gui_scripts]\noradmp = oradmp._cli:main\n", "entry_points.txt sections are ['gui_scripts']"),
        ("", "entry_points.txt sections are [], not ['console_scripts']"),
    ],
)
def test_m23_a_wrong_entry_points_file_fails(tmp_path: Path, text: str, message: str) -> None:
    errors = wheel_errors(make_wheel(tmp_path, {f"{di()}/entry_points.txt": text.encode()}))
    assert len(errors) == 1 and errors[0].startswith("wheel entry_points.txt: ") and message in errors[0], errors


def test_m24b_the_form_the_build_tool_writes_passes(tmp_path: Path) -> None:
    # maturin writes `oradmp=oradmp._cli:main` without spaces (seen in the dry build of T-18)
    entry = {f"{di()}/entry_points.txt": b"[console_scripts]\noradmp=oradmp._cli:main\n"}
    assert wheel_errors(make_wheel(tmp_path, entry)) == []


def test_m24_an_unparsable_entry_points_file_fails(tmp_path: Path) -> None:
    errors = wheel_errors(make_wheel(tmp_path, {f"{di()}/entry_points.txt": b"oradmp = oradmp._cli:main\n"}))
    assert errors == ["wheel entry_points.txt: entry_points.txt cannot be parsed"]


def test_m25_a_data_folder_with_the_script_fails_by_the_outside_rule(tmp_path: Path) -> None:
    v = version()
    errors = wheel_errors(make_wheel(tmp_path, {f"oradmp-{v}.data/scripts/oradmp": b"#!/usr/bin/python\n"}))
    assert errors == [f"wheel: oradmp-{v}.data/scripts/oradmp is outside oradmp/ and {di()}/"]


def test_m26_the_clean_pair_names_the_entry_point_in_the_listing(
    tmp_path: Path, capsys: pytest.CaptureFixture[str]
) -> None:
    assert main([str(make_wheel(tmp_path)), str(make_sdist(tmp_path))]) == 0
    assert f"{di()}/entry_points.txt" in capsys.readouterr().out


# --- T-16a2 (c), ADR-90 item 2: `--build-root` ties the build paths of the SBOM to one root.
SBOM_NAME = "sboms/oradmp-py.cyclonedx.json"


def sbom_wheel(tmp_path: Path, sbom: bytes) -> Path:
    return make_wheel(tmp_path, {f"{di()}/{SBOM_NAME}": sbom})


def test_t16a2_the_right_build_root_passes_and_the_sbom_of_make_wheel_is_below_it(tmp_path: Path) -> None:
    sbom = (
        b'{"a": "path+file:///var/tmp/oradmp-release/src/crates/oradmp-core#0.1.0",'
        b' "b": "/var/tmp/oradmp-release/src/crates/oradmp-py/src/main.rs"}\n'
    )
    errors: list[str] = []
    check_wheel(str(sbom_wheel(tmp_path, sbom)), errors, "/var/tmp/oradmp-release")
    assert errors == []


def test_t16a2_another_root_fails_with_the_byte_offset_of_each_path(tmp_path: Path) -> None:
    sbom = (
        b'{"a": "path+file:///var/tmp/oradmp-release-ci2/src/crates/oradmp-core",'
        b' "b": "/tmp/other/src/x", "c": "/var/tmp/oradmp-release/src/ok"}\n'
    )
    errors: list[str] = []
    check_wheel(str(sbom_wheel(tmp_path, sbom)), errors, "/var/tmp/oradmp-release")
    hits = [e for e in errors if SBOM_NAME in e]
    assert len(hits) == 2, errors
    assert f"byte offset {sbom.find(b'/var/tmp/oradmp-release-ci2')}" in hits[0]
    assert f"byte offset {sbom.find(b'/tmp/other')}" in hits[1]
    assert "/var/tmp/oradmp-release/src/" in hits[0]


def test_t16a2_a_root_that_is_a_prefix_of_the_string_but_not_a_folder_fails(tmp_path: Path) -> None:
    sbom = b'{"a": "/var/tmp/oradmp-release/srcx/crates"}\n'
    errors: list[str] = []
    check_wheel(str(sbom_wheel(tmp_path, sbom)), errors, "/var/tmp/oradmp-release")
    assert len([e for e in errors if SBOM_NAME in e]) == 1, errors


def test_t16a2_without_the_option_the_result_is_unchanged(tmp_path: Path) -> None:
    wheel = sbom_wheel(tmp_path, b'{"a": "path+file:///var/tmp/anywhere/src/x"}\n')
    assert wheel_errors(wheel) == []
    assert main([str(wheel), str(make_sdist(tmp_path))]) == 0


def test_t16a2_main_takes_the_option_prints_the_root_and_fails_on_another_root(
    tmp_path: Path, capsys: pytest.CaptureFixture[str]
) -> None:
    wheel, sdist = make_wheel(tmp_path), make_sdist(tmp_path)  # the SBOM of make_wheel is below /var/tmp/oradmp-release
    assert main(["--build-root", "/var/tmp/oradmp-release", str(wheel), str(sdist)]) == 0
    out = capsys.readouterr().out
    assert "check_dist: build root /var/tmp/oradmp-release\n" in out
    assert main(["--build-root", "/var/tmp/elsewhere", str(wheel), str(sdist)]) == 1
    out = capsys.readouterr().out
    assert "FAIL:" in out and "check_dist: FAILED" in out
    assert main([str(wheel), str(sdist)]) == 0
    assert "check_dist: build root (not given)\n" in capsys.readouterr().out


def test_t16a2_a_missing_value_of_the_option_is_a_usage_error(tmp_path: Path) -> None:
    assert main(["--build-root", str(make_wheel(tmp_path))]) == 2


def test_t16a2_the_build_script_passes_the_root_to_check_dist() -> None:
    text = (Path(__file__).resolve().parent.parent / "tools" / "release_build.sh").read_text()
    assert 'check_dist.py" --build-root "$ROOT"' in text


# --- T-20 (b), ADR-107 item 1: the Project-URL lines of the metadata are exactly those of pyproject.toml, in the
# wheel's METADATA and the sdist's PKG-INFO alike. Mutation (done by hand, T-20 note): a rule that compares labels
# only lets the changed-URL case pass, so that case fails.


def test_t20_pyproject_declares_the_two_urls() -> None:
    assert declared_project_urls() == {("Source", SOURCE_URL), ("Issues", ISSUES_URL)}


def test_t20_the_right_project_url_lines_pass_in_any_order(tmp_path: Path) -> None:
    def swap(t: str) -> str:
        a, b = f"Project-URL: Source, {SOURCE_URL}\n", f"Project-URL: Issues, {ISSUES_URL}\n"
        return t.replace(a + b, b + a)

    assert wheel_errors(make_wheel(tmp_path, meta_edit=swap)) == []


def test_t20_a_missing_project_url_fails_in_wheel_and_sdist(tmp_path: Path) -> None:
    line = f"Project-URL: Issues, {ISSUES_URL}\n"
    w = make_wheel(tmp_path, meta_edit=lambda t: t.replace(line, ""))
    assert wheel_errors(w) == [f"wheel METADATA: Project-URL 'Issues' '{ISSUES_URL}' is missing"]
    # the sdist's PKG-INFO is made by metadata() alone: strip the lines there
    s = make_sdist(tmp_path, {"PKG-INFO": metadata(version(), edit=lambda t: t.replace(line, ""))})
    assert sdist_errors(s) == [f"sdist PKG-INFO: Project-URL 'Issues' '{ISSUES_URL}' is missing"]


def test_t20_a_changed_project_url_fails_with_two_lines(tmp_path: Path) -> None:
    w = make_wheel(tmp_path, meta_edit=lambda t: t.replace(f"Source, {SOURCE_URL}", f"Source, {SOURCE_URL}-x"))
    assert wheel_errors(w) == [
        f"wheel METADATA: Project-URL 'Source' '{SOURCE_URL}' is missing",
        f"wheel METADATA: Project-URL 'Source' '{SOURCE_URL}-x' is not declared in pyproject.toml",
    ]


def test_t20_an_extra_project_url_fails(tmp_path: Path) -> None:
    w = make_wheel(
        tmp_path, meta_edit=lambda t: t.replace("\n\n", "\nProject-URL: Homepage, https://example.org\n\n", 1)
    )
    assert wheel_errors(w) == [
        "wheel METADATA: Project-URL 'Homepage' 'https://example.org' is not declared in pyproject.toml"
    ]
