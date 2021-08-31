const { assureMysqlDatabase } = require('./dbutils')

async function run() {
    assureMysqlDatabase(process.env.DB_NAME, 'localhost', 3306, 'root', 'mysecretpassword')
}

run()