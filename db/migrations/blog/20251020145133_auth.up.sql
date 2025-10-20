-- Types
CREATE TYPE user_role AS ENUM (
	'admin',
	'author',
	'guest'
);

-- Tables
ALTER TABLE users
ADD password text,
ADD role user_role not null default 'guest';

-- Functions
CREATE OR REPLACE FUNCTION is_admin(check_id integer)
RETURNS boolean
AS $$
BEGIN
	SELECT role = 'admin'
	FROM users
	WHERE	 id = check_id;
END;
$$ LANGUAGE plpgsql;
