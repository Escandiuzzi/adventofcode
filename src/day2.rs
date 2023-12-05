fn main() {
    //Part 1
    // let red = 12;
    // let green = 13;
    // let blue = 14;

    //let mut result = 0;

    let mut result: Vec<i32> = Vec::new();

    let input: Vec<&str> = include_str!("../resources/input_2.prod")
        .lines()
        .map(|line| line)
        .collect();

    for line in input {
        let data: Vec<&str> = line.split(":").map(|s| s).collect();
        let id = data[0].split_whitespace().collect::<Vec<&str>>()[1]
            .parse::<i32>()
            .unwrap();

        let plays: Vec<&str> = data[1].split(";").collect();

        // Part 1
        //let mut valid = true;

        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        for play in plays {
            let cubes: Vec<&str> = play.split(",").collect();

            for cube in cubes {
                let cube_data: Vec<&str> = cube.split_whitespace().collect();

                let color = cube_data[1];
                let number = cube_data[0].parse::<i32>().unwrap();

                if color == "red" && number > min_red {
                    min_red = number;
                } else if color == "green" && number > min_green {
                    min_green = number;
                } else if color == "blue" && number > min_blue {
                    min_blue = number;
                }

                // PART 1

                // if color == "red" && number > red {
                //     valid = false;
                //     break;
                // } else if color == "green" && number > green {
                //     valid = false;
                //     break;
                // } else if color == "blue" && number > blue {
                //     valid = false;
                //     break;
                // }
            }

            // PART 1
            // if !valid {
            //     break;
            // }
        }

        result.push(min_red * min_green * min_blue);

        // PART !
        // if valid {
        //     result += id;
        // }
    }

    let value: i32 = result.iter().sum();

    println!("Result {}", value);
}
