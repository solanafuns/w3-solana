package solana

import "github.com/near/borsh-go"

type PageDataEnum struct {
	Enum      borsh.Enum `borsh_enum:"true"`
	RawData   Raw
	TrunkPage TrunkPage
}

type Raw []uint8
type TrunkPage struct {
	Trunks uint8
}

func ParsePageContent(data []byte) (*PageDataEnum, error) {
	b := new(PageDataEnum)
	if err := borsh.Deserialize(b, data); err != nil {
		return nil, err
	}
	return b, nil
}
