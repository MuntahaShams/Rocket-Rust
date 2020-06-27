#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::request::Form;
use rocket::response::content::Html;

#[derive(FromForm)]
struct Number{
    number : i32,
}

#[get("/num")]
fn get_request()-> Html<String>{
    let h1=format!("
        <form style='margin:auto' action='/num' method='post'>
        <h5
        style='margin-bottom:10px; font-weight: bold; color: black;'>
        ENTER A NUMBER TO ADD IT BY 30</h5>
        <input style='padding: 5px 10px; border-radius: 2px;  height: 40px;
        width: 500px; font-size: 24px; border: 1px solid red;' type='number' 
            name='number' id='number'> </br>
            </br>
        <input style='font-size: 24px; padding:5px 12px; 
        border: none; height: 40px;
        border-radius: 3px;  color: white; background-color:red
        ;' type='Submit'>
    ");
    Html(h1)
}

#[post("/num", data = "<number>")]
fn recieve_request(number : Form<Number>) -> Html<String> {
   let h1=format!( "<p style='font-size: 35px; font-weight: bold;'>
   Given Number is = {} </br>
    and Result is ={}</p>",
    number.number, number.number+30);
    Html(h1)
}

#[get("/")]
fn home()-> Html<String>{
    let html=format!("
            <div>
            <h1
            style='margin-bottom:  10px; font-size: 4rem; font-weight: bolder; color: black;'>
            Rocket Server Application</h1>
        <a href='/num' style='padding: 10px 20px; margin-top: 10px ; 
        font-size: 24px; color: white; background-color: red; text-decoration: none; border-radius: 5px;'>Login</a>
    
        </div>");
    Html(html)
}


fn main() {
    rocket::ignite().mount("/", routes![home, get_request, recieve_request]).launch();
}



