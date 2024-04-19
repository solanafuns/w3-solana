package solana

import (
	"context"

	"github.com/blocto/solana-go-sdk/client"
	"github.com/blocto/solana-go-sdk/common"
	"github.com/blocto/solana-go-sdk/rpc"
	"github.com/sirupsen/logrus"
	"github.com/spf13/viper"
)

const (
	Devnet   Network = "devnet"
	Testnet  Network = "testnet"
	Mainnet  Network = "mainnet"
	LocalNet Network = "localnet"
)

type Network string

func (n Network) RpcUrl() string {
	switch n {
	case Devnet:
		return rpc.DevnetRPCEndpoint
	case Testnet:
		return rpc.TestnetRPCEndpoint
	case Mainnet:
		return rpc.MainnetRPCEndpoint
	case LocalNet:
		return "http://localhost:8899"
	default:
		return string(n)
	}
}

func LoadRpcClient(network Network) *client.Client {
	return client.NewClient(network.RpcUrl())
}

type W3Site struct {
	NameResolver common.PublicKey
	Program      common.PublicKey
	Client       *client.Client
}

func (s *W3Site) UrlTrunkAccount(path string, trunk_no uint8) (common.PublicKey, error) {
	logrus.Info("path: ", path)
	seeds := [][]byte{}
	if len(path) < 32 {
		seeds = append(seeds, []byte(path))
	} else {
		for i := 0; i < len(path); i += 32 {
			end := i + 32
			if end > len(path) {
				end = len(path)
			}
			seeds = append(seeds, []byte(path[i:end]))
		}
	}
	seeds = append(seeds, []byte{trunk_no})
	k, bump_seed, err := common.FindProgramAddress(seeds, s.Program)
	logrus.WithFields(logrus.Fields{
		"seeds":   seeds,
		"program": s.Program.ToBase58(),
		"bump":    bump_seed,
		"account": k.ToBase58(),
	}).Info("find url account !!!")
	return k, err
}

func (s *W3Site) UrlAccount(path string) (common.PublicKey, error) {
	logrus.Info("path: ", path)
	seeds := [][]byte{}
	if len(path) < 32 {
		seeds = append(seeds, []byte(path))
	} else {
		for i := 0; i < len(path); i += 32 {
			end := i + 32
			if end > len(path) {
				end = len(path)
			}
			seeds = append(seeds, []byte(path[i:end]))
		}
	}

	k, bump_seed, err := common.FindProgramAddress(seeds, s.Program)
	logrus.WithFields(logrus.Fields{
		"seeds":   seeds,
		"program": s.Program.ToBase58(),
		"bump":    bump_seed,
		"account": k.ToBase58(),
	}).Info("find url account !!!")
	return k, err
}

// let base_seed = ".w3-solana-name";
// let (config_pda, bump_seed) = PdaHelper::new(program_id.clone())
// .find_program_address(&[base_seed.as_bytes(), name.as_bytes()]);

func (s *W3Site) GetSiteConfig(name string) (*NameConfig, error) {
	baseSeed := ".w3-solana-name"
	seeds := [][]byte{}
	seeds = append(seeds, []byte(baseSeed))
	seeds = append(seeds, []byte(name))
	k, _, err := common.FindProgramAddress(seeds, s.NameResolver)
	if err != nil {
		return nil, err
	}
	logrus.Info(seeds)
	logrus.Info("config account is ", k.ToBase58())

	data, err := s.LoadAccountContent(&k)
	if err != nil {
		return nil, err
	}
	return ParseConfig(data)
}

func (s *W3Site) LoadAccountContent(account *common.PublicKey) ([]byte, error) {
	info, err := s.Client.GetAccountInfo(context.Background(), account.ToBase58())
	if err != nil {
		return nil, err
	}
	return info.Data, nil
}

func (s *W3Site) Network() string {
	return viper.GetString("network")
}

func GetClient() *W3Site {
	return &W3Site{
		NameResolver: common.PublicKeyFromString(viper.GetString("name_resolver")),
		Program:      common.PublicKey{},
		Client:       LoadRpcClient(Network(viper.GetString("network"))),
	}
}
