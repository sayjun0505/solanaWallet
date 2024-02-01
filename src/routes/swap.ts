import express, {Request,Response} from 'express';

const router=express.Router();
router.get('/api/swap',[],(req:Request,res:Response)=>{
    return res.send('swap')
})

export {router as swapRouter}