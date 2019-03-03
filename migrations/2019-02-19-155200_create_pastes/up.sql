CREATE TABLE pastes
(
  id            BIGSERIAL                   NOT NULL,
  filename      VARCHAR(100),
  content       text,
  creation_date timestamp without time zone NOT NULL,
  PRIMARY KEY (id)
)
  WITH (
    OIDS = FALSE
  );
