CREATE OR REPLACE FUNCTION insert_user(
    IN username VARCHAR(36),
    IN password VARCHAR(60)
) RETURNS VARCHAR(40) AS $$
DECLARE
    user_password VARCHAR(60);
    user_recovery_key VARCHAR;
BEGIN
    SELECT crypt(password, gen_salt('bf')) INTO user_password;

    user_recovery_key := encode(digest(gen_random_bytes(16), 'sha1'), 'hex');

    INSERT INTO users (user_username, user_password, user_recovery_key)
    VALUES (username, user_password, crypt(user_recovery_key, gen_salt('bf')));
    RETURN user_recovery_key;
END;
$$ LANGUAGE plpgsql;
