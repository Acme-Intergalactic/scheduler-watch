SELECT COUNT(sign_id) AS n
FROM displays_scheduleinfo AS info, signs_sign AS sign
WHERE
    info.sign_id=sign.id AND
    data_date < NOW() - '10 minutes'::interval AND 
    deleted ISNULL AND 
    external = FALSE AND 
    enabled != 0