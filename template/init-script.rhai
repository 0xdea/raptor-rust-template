// simple rhai script to set the current year
let year = system::date().year;
variable::set("year", year.to_string());
