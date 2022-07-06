describe('front-end router', () => {
    it('should display a 404 page when navigating to an unknown page', () => {
        let randomPath = `/not-real-path-${Math.floor(Math.random() * 1000)}`
        cy
            .visit(randomPath)
            .get('[data-test="404-title"]')
            .should('contain', '404')
            .get('[data-test="404-subtitle"]')
            .should('contain', "We couldn't find the page you're looking for")
            .get('[data-test="404-home-link"]')
            .should('contain', 'Take me home')
            .click()
            .url()
            .should('contain', '/')
            .and('not.contain', randomPath)
    })
})
