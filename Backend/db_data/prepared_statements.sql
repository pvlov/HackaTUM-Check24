prepare test (int) AS
    select * from dist_precalc_filter where postcode=$1;