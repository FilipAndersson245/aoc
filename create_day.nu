
def main [year: int, day: int] {
    let name = $"aoc_($year)_($day)"
    let path_year = $"./crates/($year)"
    let path_day = $"($path_year)/($day)"
    mkdir $path_year
    cp --recursive "./template" $path_year
    mv $"($path_year)/template" $path_day
    rm $"($path_day)/template" -r -f
    
    # Cookie should be something like this
    # cookie:session=xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
    let cookie = open './COOKIE.txt'
    
    def _parse_url [year: int, day: int] {
        $"https://adventofcode.com/($year)/day/($day)/input"
    }

    (curl (_parse_url $year $day) -H $cookie) | save ($"($path_day)/src/($day).txt") 
    
    fd -p crates.+(toml|rs) | lines | each { |it| 
        echo $it
        sd '§name§' $name $it
        sd '§year§' $year $it
        sd '§day§'  $day  $it
    }
}

