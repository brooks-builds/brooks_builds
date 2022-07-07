describe('authentication', () => {
    it('when logged out, we have a link that points to auth0', () => {
        cy
            .visit('/')
            .get('[data-test="auth-sign-up"]')
    })
})
