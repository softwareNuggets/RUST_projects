use chrono::{NaiveDate, Datelike};

fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}


fn days_until(start_date: &str, end_date: &str) -> (i32, i32, i32) {
	
	// end date is included 
	
	let start_date 	= NaiveDate::parse_from_str(start_date, "%Y-%m-%d").unwrap();
    let end_date 	= NaiveDate::parse_from_str(end_date, "%Y-%m-%d").unwrap();
	
	let mut days_per_month: [usize; 12] = 
			[31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    let start_year = start_date.year() as i32;
    let start_month = start_date.month() as i32;
    let start_day = start_date.day() as i32;

    let mut stop_year = end_date.year() as i32;
    let mut stop_month = end_date.month() as i32;
    let mut stop_day = end_date.day() as i32;

	if start_day > stop_day
	{
		stop_month = stop_month - 1;
		
		if is_leap_year(start_year)
		{
			days_per_month[1]=29;
		}

		stop_day = stop_day + days_per_month[(start_month-1) as usize] as i32;	
	}	

	if start_month > stop_month
	{
		stop_year = stop_year - 1;
		stop_month = stop_month+12;
	}

	let years_diff = stop_year - start_year;
	let month_diff = stop_month - start_month;
	let day_diff   = stop_day - start_day;
	
   	return (years_diff, month_diff, day_diff);
}

fn main() {
	let start_date = "2022-05-15";
    let end_date = "2023-06-12";

    let result = days_until(start_date, end_date);
	dbg!(result);
	
	assert_eq!(days_until("2020-02-01", "2020-03-01"), (0, 1, 0));
	assert_eq!(days_until("2023-05-15", "2024-05-01"), (0, 11, 17));
    	assert_eq!(days_until("2023-05-15", "2024-05-14"), (0, 11, 30));
	assert_eq!(days_until("2000-01-01", "2023-12-31"), (23, 11, 30));
}














/*
	//3 4 5 6 7 8 9 10 11 12; 13 14 15 16 17 18 19 20 21 22; 23 24 25 26 27 28 29 01
	assert_eq!(days_until("2020-02-02", "2020-03-01"), (0, 0, 28));
	
	//16 17 18 19 20 21 22 23 24 25; 26 27 28 29 30 31 01
	assert_eq!(days_until("2023-03-15", "2023-04-01"), (0, 0, 17));
	assert_eq!(days_until("2023-05-14", "2023-05-15"), (0, 0, 1));
	assert_eq!(days_until("2023-05-15", "2023-05-15"), (0, 0, 0));
    assert_eq!(days_until("2023-05-15", "2023-06-15"), (0, 1, 0));
	assert_eq!(days_until("2023-05-14", "2024-05-15"), (1, 0, 1));
	assert_eq!(days_until("2023-05-15", "2024-06-15"), (1, 1, 0));
	assert_eq!(days_until("2023-05-15", "2024-06-16"), (1, 1, 1));
	//
	//
	assert_eq!(days_until("2023-05-15", "2024-05-01"), (0, 11, 17));
    assert_eq!(days_until("2023-05-15", "2024-05-14"), (0, 11, 30));
	//
	assert_eq!(days_until("2000-01-01", "2023-12-31"), (23, 11, 30));
}
*/

