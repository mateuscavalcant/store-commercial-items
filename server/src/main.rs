use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use mysql::{prelude::Queryable, Pool, PooledConn};

// Estrutura para representar os dados do produto
#[derive(Debug, serde::Deserialize)]
struct Produto {
    nome: String,
    preco: f64,
}

// Rota para lidar com o formulário HTML
async fn formulario(_req: HttpRequest) -> impl Responder {
    // Formulário HTML simples para inserir nome e preço do produto
    format!(
        r#"
        <html>
        <head><title>Cadastro de Produtos</title></head>
        <body>
            <form action="/cadastro" method="post">
                Nome do produto: <input type="text" name="nome"><br>
                Preço do produto: <input type="number" name="preco"><br>
                <button type="submit">Cadastrar</button>
            </form>
        </body>
        </html>
    "#
    )
}

// Função para conectar ao banco de dados MySQL
fn conectar_mysql() -> Pool {
    mysql::Pool::new("mysql://usuario:senha@localhost/banco_de_dados").unwrap()
}

// Rota para lidar com o cadastro de produtos
async fn cadastro(produto: web::Form<Produto>) -> impl Responder {
    // Conectando ao banco de dados MySQL
    let pool = conectar_mysql();
    let mut conn: PooledConn = pool.get_conn().unwrap();

    // Inserindo os dados do produto no banco de dados
    conn.exec_drop(
        r"INSERT INTO produtos (nome, preco) VALUES (:nome, :preco)",
        params! {
            "nome" => &produto.nome,
            "preco" => &produto.preco,
        },
    )
    .unwrap();

    format!("Produto cadastrado com sucesso: {:?}", produto)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Configurando o servidor HTTP na porta 8080
    HttpServer::new(|| {
        App::new()
            .route("/formulario", web::get().to(formulario))
            .route("/cadastro", web::post().to(cadastro))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
