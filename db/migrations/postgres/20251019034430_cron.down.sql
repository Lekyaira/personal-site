-- pg_cron
DO $$
BEGIN
	IF EXISTS (SELECT 1 FROM pg_catalog.pg_tables WHERE tablename = 'job' AND schemaname = 'cron') THEN
		PERFORM cron.unschedule(jobid)
		FROM cron.job
		WHERE jobname =	'publish-queue-tick';
	END IF;
END;
$$;

DROP EXTENSION IF EXISTS pg_cron;
