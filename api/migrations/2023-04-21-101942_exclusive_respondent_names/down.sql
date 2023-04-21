-- This file should undo anything in `up.sql`

ALTER TABLE IF EXISTS public.availability DROP CONSTRAINT IF EXISTS availability_basic_event_name_key;