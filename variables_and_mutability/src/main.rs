const TOUCHDOWN_POINTS: i32 = 6;
fn main() {
    let season: &str = "Fall";
    let mut points_scored: i32 = 28;
    points_scored = 35;

    let event_time = "06:00";
    let event_time = 6;

    println!("My favorite season is {season}. The team scored {points_scored}. The event started at {event_time}. A touchdown worth {TOUCHDOWN_POINTS} points.");
    println!("My favorite season is {}. The team scored {}. The event started at {}. A touchdown worth {} points.",season,points_scored,event_time,TOUCHDOWN_POINTS);
    println!("My favorite season is {0}. The team scored {1}. The event started at {2}. A touchdown worth {3} points.",season,points_scored,event_time,TOUCHDOWN_POINTS);

    # [allow(unused_variables)]
    let favorite_beverage = "Snapple Apple";
}
