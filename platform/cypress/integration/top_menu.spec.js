describe("top menu bar", () => {
    it("should link to the home page", () => {
        cy
            .visit('/')
            .get('[data-test="top-menu-logo"] > a')
            .should('have.attr', 'href')
            .and('equal', '/')
    })
})
