# Question : Quel MISS TCP SSera finalement terneu par les deux hotes ? jsutifiez

L'état final est CLOSED. Il garantit que les deux flux de données indépendants (Full-Duplex) sont totalement fermés et permet aux deux hôtes de libérer définitivement les ressources système (mémoire et ports) allouées à la connexion.


# Quesiton : Un segment de donnée de 1300 octets est envoyé, Sera t'il fragmenté ? Pourquoi ?

Non, il ne sera pas fragmenté. En ajoutant les en-têtes (20 octets TCP + 20 octets IP), la taille totale atteint 1340 octets, ce qui reste inférieur à l'MTU standard de 1500 octets sur les réseaux Ethernet.


# Question : Calculez la taille totale d'un segment TCP transportant 1000 octets de donnée (sans options IP/TCP)

La taille totale est de 1040 octets.
Données : 1000 octets
En-tête TCP : 20 octets
En-tête IP : 20 octets
Calcul : 1000+20+20=1040 octets.


# Question : Si l'entête TCP inclut l'option MSS (4 octets, qu'elle .... "J'ai pas eu le temps de noter la question"