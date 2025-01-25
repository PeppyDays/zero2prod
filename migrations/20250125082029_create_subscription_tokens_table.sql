create table subscription_tokens (
    subscriber_id uuid primary key,
    token text not null
);

create unique index ix01_subscription_tokens on subscription_tokens (token);
