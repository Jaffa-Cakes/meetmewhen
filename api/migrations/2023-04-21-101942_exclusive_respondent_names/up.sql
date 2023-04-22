-- Your SQL goes here

ALTER TABLE IF EXISTS public.availability
    ADD UNIQUE (basic_event, name);