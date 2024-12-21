# sistema_Autenticacion
Crear un sistema similar al del segundo proyecto, pero utilizando el lenguaje Rust y  un framework web, como Rocket o Actix.
El proyecto Sistema de Autenticación tiene como objetivo crear una aplicación web básica utilizando Rust y un framework como Actix. El sistema permite a los usuarios registrarse e iniciar sesión de forma segura. Además, se gestionan las sesiones mediante cookies para mantener el estado de autenticación durante la navegación.

Decisiones Técnicas y Bibliotecas Utilizadas:
Rust:

Por qué usar Rust: Rust es un lenguaje de programación eficiente y seguro que permite construir aplicaciones rápidas y sin errores de memoria. La elección de Rust para este proyecto garantiza la seguridad en el manejo de datos sensibles y el alto rendimiento en el servidor.
Actix Web:

Por qué usar Actix Web: Actix Web es un framework web basado en Rust que se caracteriza por su alto rendimiento y flexibilidad. Es ideal para manejar aplicaciones web que requieren un manejo eficiente de peticiones HTTP y respuestas rápidas. Además, Actix Web tiene buenas capacidades para gestionar sesiones y cookies.
Actix Session:

Uso de actix-session: Se utilizó actix-session para gestionar las sesiones de los usuarios. Este componente permite almacenar la información de la sesión del usuario en cookies, lo que facilita la autenticación persistente entre diferentes peticiones HTTP.
Diesel:

Por qué usar Diesel: Diesel es una biblioteca ORM (Object-Relational Mapper) que facilita la interacción con bases de datos. En este proyecto, se utiliza Diesel para gestionar la base de datos PostgreSQL, creando modelos y realizando consultas de manera segura y eficiente.
bcrypt:

Por qué usar bcrypt: bcrypt es un algoritmo de hashing seguro utilizado para almacenar contraseñas de forma segura. En lugar de almacenar contraseñas en texto plano, bcrypt genera un hash único que es prácticamente imposible de revertir.
SerDe (Serialize y Deserialize):

Uso de SerDe: SerDe es una biblioteca de Rust que permite la conversión eficiente entre tipos de datos Rust y formatos JSON, lo que facilita el manejo de datos durante el registro e inicio de sesión, como la autenticación de usuarios a través de JSON.
Funcionamiento del Proyecto:
Registro de Usuario:

El usuario envía su nombre de usuario, correo electrónico y contraseña.
La contraseña se hashea utilizando bcrypt antes de ser almacenada en la base de datos.
Se crea un nuevo registro en la base de datos con los datos del usuario.
Inicio de Sesión:

El usuario ingresa su nombre de usuario y contraseña.
El sistema verifica si la contraseña proporcionada coincide con el hash almacenado en la base de datos.
Si la autenticación es exitosa, se crea una sesión y se almacena en una cookie en el navegador del usuario.
Protección de Rutas:

Algunas rutas están protegidas y solo pueden ser accedidas si el usuario ha iniciado sesión correctamente. Si el usuario no está autenticado, se redirige a la página de inicio de sesión.
Base de Datos:

Se utiliza una base de datos PostgreSQL para almacenar la información del usuario. Diesel se encarga de las interacciones con la base de datos mediante un modelo de datos en Rust.
Estructura del Proyecto:
src/main.rs: Contiene el servidor web, las rutas y la lógica de autenticación, como el registro y el inicio de sesión.
src/models.rs: Define los modelos de los datos, como el usuario y la base de datos.
templates/: Contiene los archivos HTML para las vistas.
src/schema.rs: Define el esquema de la base de datos para Diesel.
