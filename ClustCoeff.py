





















def Capprox(k,nodeindexes,AdjList): # k é um inteiro
                                    # nodeindexes são os indices dos vértices com grau>=2
                                    # adjlist é a lista de adjacência do grafo

    for i in range(1, k+1):
        # o r não é preciso porque estamos a considerar o clustering coeff
        # "simples", sem pesos
        j=random.sample(nodeindexes, 1) # escolher 1 indice dos vertices com
                                        # grau >=2; No pseudo-código isto está
                                        # como UniformRandom
        u=random.sample(Adj[j],1) # escolher 1 vertice adjacente a j (podemos
                                  # escolher este vértice a partir da linha da
                                  # lista de adj que corresponde
                                  # ao vertice j)
                                  # No pseudo-código isto está como UniformRandom
        while True:
            w=random.sample(Adjlist[j],1) # escolher outro vértice adjacente a j
            if u!=w:
                break
        if w in AdjList[u]:
                l=l+1
    return l/k


# temos de selecionar os nós com grau maior ou igual a 2, que são basicamente os
# vértices que na sua lista de adjacência têm pelo menos 2 elementos
# ter a lista de ajdacência do grafo para sabermos quais são os vértices
# adjacentes a cada vértice.
