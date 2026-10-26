-- 15_big.sql  T_BIG: 5,000,000 rows for the benchmarks and the memory test (O-17, T-15b).
-- Run as CORPUS by make_big.sh. Deterministic: no random values, the same rows every run.
-- Not a fixture: nothing from this run is committed (the dump goes to the scratch folder, /work).
whenever sqlerror exit failure
set echo on feedback on timing on serveroutput on linesize 200 pagesize 100

begin
  execute immediate 'drop table t_big purge';
exception when others then
  if sqlcode != -942 then raise; end if;
end;
/

create table t_big (
  id      number(10)    not null,
  grp     number(5),
  amount  number(12,2),
  name    varchar2(40),
  created date,
  note    varchar2(100)
);

-- 2,500 x 2,000 = 5,000,000 rows (two small row sources instead of one big CONNECT BY, which can run out of memory)
insert /*+ append */ into t_big
select (a.n - 1) * 2000 + b.n                                              as id,
       mod((a.n - 1) * 2000 + b.n, 1000)                                   as grp,
       mod(((a.n - 1) * 2000 + b.n) * 7919, 1000003) / 100                 as amount,
       'name_' || lpad((a.n - 1) * 2000 + b.n, 8, '0')                     as name,
       date '2020-01-01' + mod((a.n - 1) * 2000 + b.n, 1500)               as created,
       case when mod((a.n - 1) * 2000 + b.n, 10) = 0 then null
            else rpad('n' || to_char(mod((a.n - 1) * 2000 + b.n, 9973)),
                      20 + mod((a.n - 1) * 2000 + b.n, 60), 'x') end       as note
from (select level n from dual connect by level <= 2500) a,
     (select level n from dual connect by level <= 2000) b;
commit;

-- a small truth file with totals (checked by hand and by the benchmark note), not per-row truth
set echo off feedback off timing off heading off pagesize 0 trimspool on
spool /work/truth_big.txt
select 'rows='        || count(*)                       from t_big;
select 'sum_id='      || sum(id)                        from t_big;
select 'sum_amount='  || sum(amount)                    from t_big;
select 'null_notes=' || (count(*) - count(note)) from t_big;
select 'min_created=' || to_char(min(created), 'YYYY-MM-DD') from t_big;
select 'max_created=' || to_char(max(created), 'YYYY-MM-DD') from t_big;
spool off
exit
