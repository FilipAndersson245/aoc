# def main [year, day] {
#     let name = $"aoc_($year)_($day)"
#     let path_year = $"./crates/($year)"
#     let path_day = $"($path_year)/($day)"
#     mkdir $path_year
#     cp --recursive "./template" $path_year
#     mv $"($path_year)/template" $path_day
#     rm $"($path_day)/template" -r -f
    
#     # Cookie should be something like this
#     # cookie:session=xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
#     let cookie = open './COOKIE.txt'
    
#     def _parse_url [year: int, day: int] {
#         $"https://adventofcode.com/($year)/day/($day)/input"
#     }

#     (curl (_parse_url $year $day) -H $cookie) | save ($"($path_day)/src/($day).txt") 
    
#     fd -p crates.+(toml|rs) | lines | each { |it| 
#         echo $it
#         sd '§name§' $name $it
#         sd '§year§' $year $it
#         sd '§day§'  $day  $it
#     }
# }

def aoc-create-day [year, day] {
    let _ = (aoc-get-day $year $day)
    let name = $"aoc_($year)_($day)"
    let path_year = $"./crates/($year)"
    let path_day = $"($path_year)/($day)"
    mkdir $path_year
    cp --recursive "./template" $path_year
    mv $"($path_year)/template" $path_day
    rm $"($path_day)/template" -r -f

    fd -p crates.+(toml|rs) | lines | each { |it| 
        echo $it
        sd '§name§' $name $it
        sd '§year§' $year $it
        sd '§day§'  $day  $it
    }
}

def aoc-get-day [year, day] {
    if ("~/.aoc/" | path exists) {} else {
        mkdir ("~/.aoc/" | path expand)
    }

    let year_folder = ($"~/.aoc/($year)/" | path expand)
    if ($year_folder | path exists) {} else {
        mkdir $year_folder
    }

    let save_path = ($"~/.aoc/($year)/($day).txt" | path expand)
    if ($save_path | path exists) {} else {
        let url = $"https://adventofcode.com/($year)/day/($day)/input"
        let cookie = $"session=($env.AOC_COOKIE)"
        let data = (fetch $url --headers ["cookie" $cookie] | str trim)
        $data | save -r $save_path ;
        echo $"fetching year: ($year) day: ($day) to ($save_path)" ;
    }
}

def aoc-get-year [year] {
    for day in 1..25 {
        let _ = (aoc-get-day $year $day)
    }
}

def aoc-get-all [] {
    let current_year = (date now | date format %Y | into int)
    let current_month = (date now | date format %m | into int)
    let current_day = (date now | date format %d | into int)
    for year in 2015..($current_year - 1) {
        let _ = (aoc-get-year $year)
    }
    if ($current_month == 12) {
        for day in 1..$current_day {
            let _ = (aoc-get-day $current_year $day)
        }
    } else {}
    echo "Done"
}