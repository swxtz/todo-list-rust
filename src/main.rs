
mod db;

use db::database;

fn main() {
    database::create_db();
}
