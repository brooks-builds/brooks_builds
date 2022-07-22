describe('authentication', () => {
    it('when logged out, we have a link that points to auth0', () => {
        cy
            .login()
            .visit('/')
            .wait(100)
            .get('[data-test="auth-sign-up"]')
            .should('not.exist')
    })
})
