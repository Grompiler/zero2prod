-- Add migration script here
INSERT INTO users (user_id, username, password_hash)
VALUES (
    '0d0d7e6c-5632-4bd0-b3af-1e198f7b5662',
    'admin',
    '$argon2id$v=19$m=15000,t=2,p=1$LR89REJcvCmMkR5bVMYd0A$k8ZSpwljOKHcZYEqrIjz/xvH4ytAK4DoE0k+yo0DHDo'
)

