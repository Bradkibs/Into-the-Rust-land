fn greet_world(){
    let english = "Hello World";
    let japanese = "ハロー・ワールド";
    let swahili = "Habari yenu walimwengu";
    let languages = [english, japanese, swahili];
    for language in languages.iter(){
        println!("{}", &language);
    }
}
fn main(){
    greet_world();
}