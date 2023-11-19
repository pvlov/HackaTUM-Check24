create unlogged table dist_precalc_filter (
    postcode int not null,
    postcode_extension_distance_group pedg not null,
    id int not null,
    max_driving_distance int,
    dist double precision not null,
    primary key (postcode, id)
);

with tmp as (select postcode,
                    postcode_extension_distance_group,
                    id,
                    max_driving_distance,
                    st_distancesphere(st_makepoint(pc.lon, pc.lat), st_makepoint(sp.lon, sp.lat)) as dist
             from postcode_opt pc,
                  profiles_pos_max_dist sp)
insert into dist_precalc_filter
select *
from tmp
where dist < max_driving_distance
order by postcode, dist