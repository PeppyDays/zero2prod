begin;
    update subscribers set status = 'Confirmed' where status is null;
    alter table subscribers alter column status set not null;
commit;
