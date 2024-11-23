-- Add migration script here
-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp" WITH SCHEMA public;

-- Create or replace functions first
CREATE OR REPLACE FUNCTION public.update_updated_at_column() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$;

CREATE OR REPLACE FUNCTION public.import_schedule(p_numeric_id integer, p_date date, p_tasks jsonb) RETURNS void
    LANGUAGE plpgsql
    AS $$
  DECLARE
      v_user_uuid uuid;
      v_task JSONB;
      v_completed_count INTEGER := 0;
      v_total_count INTEGER;
  BEGIN
      -- Get the UUID for the numeric ID
      SELECT id INTO v_user_uuid
      FROM users u
      INNER JOIN profiles p ON p.user_id = u.id
      WHERE p.user_id IN (
        SELECT uuid
        FROM id_mapping
        WHERE numeric_id = p_numeric_id
      );

      IF v_user_uuid IS NULL THEN
          RAISE EXCEPTION 'No UUID found for numeric_id %', p_numeric_id;
      END IF;

      -- Count total and completed tasks
      SELECT count(*), count(*) FILTER (WHERE (value->>'completed')::boolean)
      INTO v_total_count, v_completed_count
      FROM jsonb_array_elements(p_tasks);

      -- Insert schedule
      INSERT INTO schedules (user_id, schedule_date, completed_tasks, total_tasks)
      VALUES (v_user_uuid, p_date, v_completed_count, v_total_count)
      ON CONFLICT (user_id, schedule_date) DO UPDATE SET
          completed_tasks = EXCLUDED.completed_tasks,
          total_tasks = EXCLUDED.total_tasks,
          updated_at = CURRENT_TIMESTAMP;

      -- Insert tasks
      FOR v_task IN SELECT value FROM jsonb_array_elements(p_tasks)
      LOOP
          INSERT INTO tasks (
              user_id,
              schedule_date,
              name,
              category,
              start_time,
              end_time,
              completed
          )
          VALUES (
              v_user_uuid,
              p_date,
              v_task->>'name',
              v_task->>'category',
              (p_date || ' ' || (v_task->>'startTime'))::TIMESTAMP WITH TIME ZONE,
              (p_date || ' ' || (v_task->>'endTime'))::TIMESTAMP WITH TIME ZONE,
              (v_task->>'completed')::boolean
          );
      END LOOP;
  END;
  $$;

CREATE OR REPLACE FUNCTION public.import_user_profile(p_old_id integer, p_cluster integer, p_scores jsonb, p_preferences text[]) RETURNS void
    LANGUAGE plpgsql
    AS $$
      BEGIN
          INSERT INTO profiles (user_id, cluster, personality_scores, preferences)
          SELECT
              m.uuid,
              p_cluster,
              p_scores,
              p_preferences
          FROM id_mapping m
          WHERE m.old_id = p_old_id;
      END;
      $$;

-- Create tables if they don't exist (this won't affect existing tables)
CREATE TABLE IF NOT EXISTS public.users (
    id uuid NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    CONSTRAINT users_pkey PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS public.profiles (
    user_id uuid NOT NULL,
    cluster integer DEFAULT 0 NOT NULL,
    preferences text[],
    personality_scores jsonb NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    CONSTRAINT profiles_pkey PRIMARY KEY (user_id),
    CONSTRAINT profiles_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS public.schedules (
    user_id uuid NOT NULL,
    schedule_date date NOT NULL,
    completed_tasks integer DEFAULT 0 NOT NULL,
    total_tasks integer DEFAULT 0 NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    CONSTRAINT schedules_pkey PRIMARY KEY (user_id, schedule_date),
    CONSTRAINT schedules_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS public.tasks (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    user_id uuid,
    schedule_date date NOT NULL,
    name character varying(255) NOT NULL,
    category character varying(100) NOT NULL,
    start_time timestamp with time zone NOT NULL,
    end_time timestamp with time zone NOT NULL,
    completed boolean DEFAULT false NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    CONSTRAINT tasks_pkey PRIMARY KEY (id),
    CONSTRAINT tasks_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE,
    CONSTRAINT tasks_user_id_schedule_date_fkey FOREIGN KEY (user_id, schedule_date)
        REFERENCES public.schedules(user_id, schedule_date) ON DELETE CASCADE
);

-- Create indexes if they don't exist
CREATE INDEX IF NOT EXISTS idx_tasks_user_date ON public.tasks(user_id, schedule_date);
