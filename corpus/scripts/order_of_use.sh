export ORACLE_PASSWORD='q1w2e3r4'
cd ~/oradump/corpus/scripts

docker rm -f ora18 ora11            # old containers have the wrong mounts; everything is reproducible
./docker_run.sh 18c
./docker_run.sh 11g
./db_setup.sh 18c
./db_setup.sh 11g
./make_fixtures.sh 18c
./make_fixtures.sh 11g
./make_work.sh 18c
./make_work.sh 11g