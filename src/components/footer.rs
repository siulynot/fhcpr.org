use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    let current_year = 2025; // In a real application, this would be dynamically generated
    
    html! {
        <footer class="bg-dark text-white py-4 mt-auto">
            <div class="container">
                <div class="row">
                    <div class="col-md-4 mb-4 mb-md-0">
                        <h5>{"First Home Care Center, Inc."}</h5>
                        <p class="small">
                            {"Una agencia sin fines de lucro dedicada a ofrecer servicios de salud de calidad en la comodidad de tu hogar."}
                        </p>
                    </div>
                    
                    <div class="col-md-4 mb-4 mb-md-0">
                        <h5>{"Enlaces Rápidos"}</h5>
                        <ul class="list-unstyled">
                            <li><a href="#" class="text-white text-decoration-none">{"Inicio"}</a></li>
                            <li><a href="#servicios" class="text-white text-decoration-none">{"Servicios"}</a></li>
                            <li><a href="#requisitos" class="text-white text-decoration-none">{"Requisitos"}</a></li>
                            <li><a href="#sobre-nosotros" class="text-white text-decoration-none">{"Sobre Nosotros"}</a></li>
                            <li><a href="#contacto" class="text-white text-decoration-none">{"Contacto"}</a></li>
                        </ul>
                    </div>
                    
                    <div class="col-md-4">
                        <h5>{"Contacto"}</h5>
                        <ul class="list-unstyled">
                            <li>{"#96 Oeste, Calle de la Candelaria"}</li>
                            <li>{"Mayagüez, 00680"}</li>
                            <li>{"Tel: 787-834-2295"}</li>
                            <li>{"Email: admin@fhcpr.org"}</li>
                        </ul>
                    </div>
                </div>
                
                <hr class="my-4" />
                
                <div class="row">
                    <div class="col-md-6 text-center text-md-start">
                        <p class="small mb-0">
                            {format!("© {} First Home Care Center, Inc. Todos los derechos reservados.", current_year)}
                        </p>
                    </div>
                    <div class="col-md-6 text-center text-md-end">
                        <a href="#" class="text-white text-decoration-none me-3">{"Política de Privacidad"}</a>
                        <a href="#" class="text-white text-decoration-none">{"Términos de Servicio"}</a>
                    </div>
                </div>
            </div>
        </footer>
    }
}
