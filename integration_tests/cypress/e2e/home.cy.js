describe('home', () => {
    it('should have a title', () => {
        cy.visit('/')
            .get('[data-test="landing-title"]')
            .should('contain', 'Welcome to Brooks Builds')
    })
})
