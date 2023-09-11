# Polkadot Hub - ink! en Español

## Trabajo Práctico - Enunciado final

El objetivo del trabajo práctico es crear una **plataforma de gestión de reputación** según las contribuciones realizadas a una organización.

### Reglas

#### Organización
- Una **organización** tiene miembros.
- Los miembros tienen roles: **Admin** o **Contributor**.
- Los **contributors** participan haciendo **aportes off-chain**. 
- Los aportes off-chain se **valorizarán** mediante votos on-chain entre contributors.
- La organización, **mediante su Admin**, abrirá **rondas de votación con una duración determinada que podrá variar entre las diferentes rondas**.
- Al momento de crear la ronda de votación, el Admin deberá indicar:
  - **el monto de fondos a repartir** entre los contributors.
  - **la cantidad de votos** que podrá efectuar cada uno de ellos.
- **Los fondos deberán ser cargados por el Admin.**

#### Votación
- Los contributors podrán votar de forma **positiva** o **negativa** a otros contributors. 
- Estos votos **impactarán en el valor de reputación** del contributor votado.
- **El valor de reputación de un contributor nunca podrá ser menor a 1**.
- El poder de voto de cada contributor será **proporcional a su valor de reputación**. *La fórmula quedará a criterio de cada uno*.
  - Ejemplo: f(member_pts, target_pts, value) = target_pts + value * sqrroot(member_pts)
  - Value = +PTS o -PTS

#### Premiación
- Al finalizar la ronda de votación, **los fondos se repartirán entre los contributors en base a su valor de reputación** a partir de una transacción ejecutada por el Admin.
- Luego de que se repartan los fondos, **los valores de reputación se resetearán**.
- Se **entregarán badges (NFTs)** a los 3 contributors con mayor valor de reputación (Gold, Silver and Bronze).


### Entregable

- Se deberá presentar un repositorio de código con los contratos.
- El README del repo deberá contener la explicación de la solución.