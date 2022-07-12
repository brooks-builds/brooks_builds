describe('authentication', () => {
    it('when logged out, we have a link that points to auth0', () => {
        cy
            .clearCookies()
            .visit('/')
            .get('[data-test="auth-sign-up"]')
            .click()
            .url()
            .should("not.contain", "http://localhost:8080")
            .visit('/')
            .getCookie("auth0_state")
            .should('have.property', 'domain', 'http://localhost:8080')
            .and('have.property', 'expiry', '???')
    })
})
