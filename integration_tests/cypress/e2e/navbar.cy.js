describe('navbar', () => {
  it('has a logo', () => {
    cy.visit('/')
      .get('[data-test="nav-logo"]');
  });

  it('has the title of the web app', () => {
    cy.visit('/')
      .get('[data-test="nav-title"]')
      .should('contain', 'Brooks Builds')
  })
})
