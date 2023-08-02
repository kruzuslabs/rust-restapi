-- Drop the foreign key constraint from the resources table first
ALTER TABLE resources
DROP CONSTRAINT IF EXISTS author_id;


ALTER TABLE users
DROP CONSTRAINT IF EXISTS id;

-- Now, you can drop the resources table
DROP TABLE IF EXISTS "resources";

-- Finally, drop the users table
DROP TABLE IF EXISTS "users";
