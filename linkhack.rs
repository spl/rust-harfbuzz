// FIXME: This is a huge hack to find the static version of the library
// instead of the shared. It looks in a very specific place that only has
// relevance to servo.
#[link_args = "../harfbuzz/src/.libs/libharfbuzz.a -lpango-1.0"]
#[no_link]
native mod m { }
