## CAA - Labo2



**Auteur:** Chris Barros Henriques



L'application est un coffre permettant de stocker des fichiers chiffrés.



### Architecture



#### Argon2

Pour la dérivation de clés afin de permettre le chiffrement des fichiers, j'ai choisi Argon2 (plus précisément Argo2id) afin d'avoir un bon compromis entre sécurité contre les CPU attacks et les side-channel attacks.



Le choix des constantes est une partie importante et j'ai appliqué la méthode enseignée dans le cours qui est:

```
1. Select the number of parallel threads. 
2. Fix the number of memory that can be used.
3. Based on application, decide on how long users can wait (website : 1s, desktop login : 5s, infrequent login : >20s) 
4. Raise complexity until it reached maximum allowed time
```



Je suis donc arrivé à:

**Degré de parallélisation:** 4 threads lancés simultanément

**Utilisation de la mémoire:** 2GB

**Taille de sel:** 128 bits

**Temps d'exécution:** 1s car considéré comme une web app



#### Authentification

Il était demandé d'effectuer une authentification basée sur un challenge-response. Voici donc le protocole:



- Le client envoie son nom d'utilisateur au serveur
- Si le serveur a cet utilisateur dans sa base données, il lui envoie un nonce ainsi qu'un sel pour permettre la dérivation de clé du MAC.