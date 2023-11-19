CREATE TABLE profiles_pos_max_dist (
  id serial NOT NULL,
  lon double precision DEFAULT NULL,
  lat double precision DEFAULT NULL,
  max_driving_distance int DEFAULT NULL,
  PRIMARY KEY (id)
);

CREATE TABLE profiles_score_info (
    id                    serial                 not null primary key,
    picture_score         double precision       not null,
    description_score     double precision       not null,
    first_name varchar(255) DEFAULT NULL,
    last_name varchar(255) DEFAULT NULL,
    city varchar(255) DEFAULT NULL,
    street varchar(255) DEFAULT NULL,
    house_number varchar(255) DEFAULT NULL
);

insert into profiles_pos_max_dist
select id, lon, lat, max_driving_distance
from service_provider_profile;

insert into profiles_score_info
select id, profile_picture_score, profile_description_score, first_name, last_name, city, street, house_number
from service_provider_profile
    join quality_factor_score on id = profile_id;
