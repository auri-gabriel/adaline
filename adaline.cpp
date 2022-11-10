#include <iostream>
#include <string>
#include <cmath>
#include <algorithm>
// (c) Marcelo Thielo 1997
using namespace std;
class Adaline
{
// Veja http://www.learnartificialneuralnetworks.com/perceptronadaline.html
private:
    double f(double arg)
    {
        return tanh(arg*100.0);
    }
    double w[2];
    double x[4][2];
    double t[4];
    double b;
    double y;
    double soma;
    double eta;

public:
    Adaline();
    ~Adaline();
    void Treinamento(int maxiterates);
    double Propaga(int i);
    void Atualiza_Pesos(int i, double y_in);
    void Atualiza_Bias(int i, double y_in);
    void Apresenta_Resultados(void);
    void Cria_Treinamento(const double * A)
    {
        for (int i=0; i<4; i++)t[i]=A[i];
    }
};

Adaline::~Adaline() {}

Adaline::Adaline()
{

    w[0]=0.0;
    w[1]=0.0;

    x[0][0]=-1;
    x[0][1]=-1;
    x[1][0]=1;
    x[1][1]=1;
    x[2][0]=-1;
    x[2][1]=1;
    x[3][0]=1;
    x[3][1]=-1;

    b = 0;
    y = 0;
    eta = 0.1;
    soma = 0.0;
}
double Adaline::Propaga(int i)
{
    double soma = 0;
    for (int j=0; j<2; j++)
        soma += x[i][j] * w[j];
    return ( soma + b);
}
void Adaline::Atualiza_Pesos(int i, double y_res)
{
    for (int j=0; j<2; j++)
        w[j] += eta * (t[i] - y_res) * x[i][j];
}
void Adaline::Atualiza_Bias(int i, double y_res)
{
    b+=(t[i]-y_res)*eta;
}

void Adaline::Apresenta_Resultados()
{
    for (int l=0; l<4; l++)
        cout << l << "-th x[0]=" << x[l][0]<<" x[1]=" << x[l][1] <<" saida=" << f(Propaga(l))<< endl;
}
void Adaline::Treinamento(int maxiterates)
{

    double y_interm;

    for (int k=1; k<maxiterates; k++)
    {

        int hits = 0;

        cout << "It= " << k << endl;

        for (int i=0; i<4; i++)
        {

            y_interm=Propaga(i);

            y=f(y_interm);

            Atualiza_Pesos(i,y_interm);

            if (y == t[i]) hits+=1;
            else Atualiza_Bias(i, y_interm);
        }
        if (hits == 4)
        {
            cout <<"Aprendizado concluido com " << k << " iteracoes" << endl;
            break;
        }
    }
}
int main (void)
{
    double XOR[]= {-1,-1,1,1};
    double OR[]= {-1,1,1,1};
    Adaline ada1;
    ada1.Cria_Treinamento(OR);
    ada1.Treinamento(1000000);
    ada1.Apresenta_Resultados();
    cout << "Fim";
    exit(0);

}