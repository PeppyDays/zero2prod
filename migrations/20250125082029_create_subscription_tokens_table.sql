create table subscription_tokens (
    token text primary key,
    subscriber_id uuid not null
);
