enum HttpStatus {
    Ok = 200,
    NotFound = 404,
    InternalServerError = 500,
}

fn handle_response(status: HttpStatus) {
    match status {
        HttpStatus::Ok => println!("Request successful"),
        HttpStatus::NotFound => println!("Resource not found"),
        HttpStatus::InternalServerError => println!("Server error occurred"),
    }
}

fn main() {
    handle_response(HttpStatus::NotFound);
}