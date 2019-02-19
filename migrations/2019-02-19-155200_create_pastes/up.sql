CREATE TABLE pastes
(
  id            varchar(20)                 NOT NULL,
  filename      VARCHAR(100),
  content       text,
  creation_date timestamp without time zone NOT NULL,
  PRIMARY KEY (id)
)
  WITH (
    OIDS = FALSE
  );


ALTER TABLE pastes
  OWNER TO pastepin_user;

GRANT ALL ON TABLE pastes TO postgres;
GRANT ALL ON TABLE pastes TO pastepin_user;
