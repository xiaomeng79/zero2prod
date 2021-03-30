-- Add migration script here
BEGIN;
    UPDATE subscriptions
        SET status = 'confirmed'
        where STATUS IS NULL;
    ALTER TABLE subscriptions ALTER COLUMN status SET NOT NULL;
COMMIT;