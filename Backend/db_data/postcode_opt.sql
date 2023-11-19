create type pedg AS ENUM ('group_a', 'group_b', 'group_c');

create table postcode_opt
(
    postcode                          int                    not null primary key,
    lon                               double precision       not null,
    lat                               double precision       not null,
    postcode_extension_distance_group pedg default 'group_a' not null,
    created_at                        timestamp(6)           null /*comment '(DC2Type:datetime_immutable)'*/,
    updated_at                        timestamp(6)           null /*comment '(DC2Type:datetime_immutable)'*/
);

insert into postcode_opt
select cast(postcode as int), lon, lat, cast(postcode_extension_distance_group as pedg), created_at, updated_at
from postcode;