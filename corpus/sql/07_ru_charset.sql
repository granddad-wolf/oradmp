-- 07_ru_charset.sql: run as SYSDBA inside the throwaway container ora11ru, on an EMPTY database.
-- Switches the database character set from AL32UTF8 to CL8MSWIN1251 with the internal (unsupported)
-- command that skips the superset check. Never use this on a database that holds real data.
WHENEVER SQLERROR EXIT FAILURE
SHUTDOWN IMMEDIATE
STARTUP MOUNT
ALTER SYSTEM ENABLE RESTRICTED SESSION;
ALTER SYSTEM SET JOB_QUEUE_PROCESSES=0 SCOPE=MEMORY;
ALTER SYSTEM SET AQ_TM_PROCESSES=0 SCOPE=MEMORY;
ALTER DATABASE OPEN;
ALTER DATABASE CHARACTER SET INTERNAL_USE CL8MSWIN1251;
SHUTDOWN IMMEDIATE
STARTUP
EXIT
