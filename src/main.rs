static la_liga_goals: u32 = 43;
static champions_league_goals: u32 = 10;
static copa_del_rey_goals: u32 = 5;

static total_goals: u32 = la_liga_goals + champions_league_goals + copa_del_rey_goals;

fn main() {
   
    
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(total_goals, 58, "total goals should equal to 58");
    }
}
