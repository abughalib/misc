const mysql = require('mysql');

let con = mysql.createConnection({
    host: "localhost",
    user: "root",
    password: '',
    database: 'users'
})
con.connect((err)=>{
    if(err) throw err;
    console.log("Server Connected!")
    let name = 'abugh'
/*    con.query('CREATE DATABASE users', (err)=>{
        if(err) throw err;
        console.log('database created!');
    } );*/
/*    con.query(`CREATE TABLE users(id int primary key auto_increment not null, password text not null, username varchar(50) not null, salt text not null);`, (err, result)=>{
        if(err) throw err;
        console.log(`Created Table`)
    })*/
    let data = {
        username: 'abugh',
        password: `password`,
        salt: 'someId'
    }
    //con.query(`INSERT INTO users(username, password, salt) values('${data.username}', '${data.password}', '${data.salt}')`)
    con.query(`SELECT * FROM users where username="${name}"`, (err, result)=>{
        if(err) throw err;
        for(let i = 0; i < result.length; i++){
            console.log(result)
        }
    })
})
