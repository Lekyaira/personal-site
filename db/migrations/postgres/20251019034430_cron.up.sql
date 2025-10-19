-- pg_cron
CREATE EXTENSION pg_cron;
SELECT cron.schedule('publish-queue-tick', '* * * * *', $$SELECT tick_publish_queue();$$);
