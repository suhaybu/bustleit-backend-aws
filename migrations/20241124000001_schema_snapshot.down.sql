-- Drop all tables and functions (this is a full schema revert)
DROP TABLE IF EXISTS public.tasks CASCADE;
DROP TABLE IF EXISTS public.schedules CASCADE;
DROP TABLE IF EXISTS public.profiles CASCADE;
DROP TABLE IF EXISTS public.users CASCADE;
DROP FUNCTION IF EXISTS public.import_schedule(integer, date, jsonb);
DROP FUNCTION IF EXISTS public.import_user_profile(integer, integer, jsonb, text[]);
DROP FUNCTION IF EXISTS public.update_updated_at_column();
DROP EXTENSION IF EXISTS "uuid-ossp";
