// ***********************************************
// This example commands.js shows you how to
// create various custom commands and overwrite
// existing commands.
//
// For more comprehensive examples of custom
// commands please read more here:
// https://on.cypress.io/custom-commands
// ***********************************************
//
//
// -- This is a parent command --
// Cypress.Commands.add('login', (email, password) => { ... })
//
//
// -- This is a child command --
// Cypress.Commands.add('drag', { prevSubject: 'element'}, (subject, options) => { ... })
//
//
// -- This is a dual command --
// Cypress.Commands.add('dismiss', { prevSubject: 'optional'}, (subject, options) => { ... })
//
//
// -- This will overwrite an existing command --
// Cypress.Commands.overwrite('visit', (originalFn, url, options) => { ... })

Cypress.Commands.add('login', () => {
    const options = {
        url: Cypress.env('auth0_login_url'),
        method: "POST",
        body: {
            audience: Cypress.env('auth0_audience'),
            grant_type: 'password',
            client_id: Cypress.env('auth0_client_id'),
            client_secret: Cypress.env('auth0_client_secret'),
            scope: 'openid profile email',
            username: Cypress.env('auth0_username'),
            password: Cypress.env('auth0_password'),
        },
        log: true
    };

    cy
        .visit('/')
        .request(options).then(response => {
            const {access_token} = response.body;
            cy.setCookie('auth0_token', access_token, {log: false});
            cy.log('cookie set');
        });
});
